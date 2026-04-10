FROM node:22-slim

WORKDIR /app

COPY package.json .
RUN npm install --omit=dev

COPY calculator.js .

EXPOSE 3000

CMD ["node", "calculator.js"]
