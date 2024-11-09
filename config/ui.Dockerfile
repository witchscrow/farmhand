# Build stage
FROM --platform=$BUILDPLATFORM node:20-slim as builder

# Set working directory
WORKDIR /app

# Copy package files
COPY services/barn-ui/package.json services/barn-ui/yarn.lock ./

# Install dependencies
RUN yarn install --frozen-lockfile

# Copy source files
COPY services/barn-ui .

# Build the application
RUN yarn build

# Runtime stage
FROM --platform=$TARGETPLATFORM node:20-slim

# Set working directory
WORKDIR /app

# Copy package files for production dependencies
COPY services/barn-ui/package.json services/barn-ui/yarn.lock ./

# Install production dependencies only
RUN yarn install --production --frozen-lockfile

# Copy built application from builder
COPY --from=builder /app/build ./build

# Expose default SvelteKit port
EXPOSE 3000

# Start the application
CMD ["node", "build"]
