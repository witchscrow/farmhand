# Build stage
FROM --platform=$BUILDPLATFORM node:22-slim as builder

# Set working directory
WORKDIR /app

# Copy root package.json and yarn.lock
COPY package.json yarn.lock ./

# Copy UI package.json
COPY services/barn-ui/package.json services/barn-ui/
COPY services/barn-ui/yarn.lock services/barn-ui/

# Install all dependencies (including dev dependencies)
RUN yarn install

# Copy UI source code
COPY services/barn-ui/ services/barn-ui/

# Build the UI application
RUN yarn workspace web build

# Runtime stage
FROM --platform=$TARGETPLATFORM node:22-slim

# Set working directory
WORKDIR /app

# Copy only production dependencies
COPY --from=builder /app/services/barn-ui/package.json ./
COPY --from=builder /app/services/barn-ui/yarn.lock ./
RUN yarn install --production

# Copy built application
COPY --from=builder /app/services/barn-ui/build ./build

# Expose port
EXPOSE 3000

# Start the application
CMD ["node", "build"]
