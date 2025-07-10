
FROM node:lts-alpine as builder

WORKDIR /app

COPY package.json package-lock.json ./


RUN npm ci

COPY . .

RUN npm run build


FROM node:lts-alpine

WORKDIR /app

COPY --from=builder /app/dist ./dist

RUN npm install -g serve

EXPOSE 3000

CMD [ "npx", "serve", "-s", "dist", "-l", "3000" ]
