FROM node:alpine3.11


WORKDIR /app
COPY /src /app/src
COPY package*.json /app/
COPY . .

RUN npm install
RUN npm run build

EXPOSE 5173

CMD ["npm", "run", "dev"]