import "dotenv/config";
import express from "express";
import logger from "./log/logger";
import { graphqlHTTP } from "express-graphql";
import { buildSchema } from "graphql";

const app = express();
const port = process.env.PORT;

var schema = buildSchema(`
  type Query {
    hello: String
  }
`);

var root = {
  hello: () => {
    return 'Hello World';
  },
};

app.use('/graphql', graphqlHTTP({
  schema: schema,
  rootValue: root,
  graphiql: true
}));

app.listen(port, () => {
  logger.log("info", "Server listening to port " + port);
});
