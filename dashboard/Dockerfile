ARG NODE_VERSION=24
FROM node:${NODE_VERSION}-alpine AS builder
WORKDIR /app

RUN npm install -g vite

COPY . .

RUN npm ci
RUN npm run build

FROM node:${NODE_VERSION}-alpine
WORKDIR /app
ENV NODE_ENV=production

COPY package*.json ./
RUN npm ci --omit=dev

# copy build folders
COPY --from=builder /app/build ./build
COPY --from=builder /app/.svelte-kit ./.svelte-kit

RUN chown -R node:node /app
USER node

EXPOSE 3000
CMD ["node", "build"]