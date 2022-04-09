import 'dotenv/config' 
import express from 'express';
import winston from 'winston';

const app = express();
//please
const port = process.env.PORT;

// creating the logger of the app
const format = winston.format;
const transports = winston.transports;

const logger = winston.createLogger({
    level: 'info',
    format: format.combine(
        format.timestamp({
            format: 'YYYY-MM-DD HH:mm:ss'
          }),
          format.errors({ stack: true }),
          format.splat(),
          format.json()

    ),
    defaultMeta: { service: 'api-server' },
    transports: [
      //
      // - Write all logs with importance level of `error` or less to `error.log`
      // - Write all logs with importance level of `info` or less to `combined.log`
      //
      new transports.File({ filename: './log/error.log', level: 'error' }),
      new transports.File({ filename: './log/combined.log' }),
    ],
  });

//
// If we're not in production then log to the `console` with the format:
// `${info.level}: ${info.message} JSON.stringify({ ...rest }) `
//
if (process.env.NODE_ENV !== 'production') {
    logger.add(new transports.Console({
      format: format.combine(
          format.colorize(),
          format.simple()
      ),
    }));
  }

app.get('/', (req, res) => {
    res.send('Hello World !');
});

app.listen(port, () => {
    logger.log('info', 'pleeease ? App listening to port ' + port);
});