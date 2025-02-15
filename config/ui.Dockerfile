# Build stage
FROM --platform=$BUILDPLATFORM node:22-slim as builder

# Set working directory
WORKDIR /app

# Copy root package.json and yarn.lock
COPY package.json package.json
COPY yarn.lock yarn.lock
COPY web/package.json web/package.json
COPY web/yarn.lock web/yarn.lock

# Install all dependencies (including dev dependencies)
RUN yarn install

# Copy UI source code
COPY web/ web/

# Build the UI application
RUN yarn workspace web build

# Runtime stage
FROM --platform=$TARGETPLATFORM node:22-slim

# Set working directory
WORKDIR /app

# Copy only production dependencies
COPY --from=builder /app/web/package.json ./
COPY --from=builder /app/web/yarn.lock ./
RUN yarn install --production

# Copy built application
COPY --from=builder /app/web/build ./build

# Expose port
EXPOSE 3000

# Start the application
CMD ["node", "build"]
