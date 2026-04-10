FROM node:18-alpine

WORKDIR /app

COPY package.json ./
RUN npm install

COPY calculator.js ./

EXPOSE 3000

CMD ["node", "calculator.js"]
