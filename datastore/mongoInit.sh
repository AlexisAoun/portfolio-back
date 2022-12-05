#!/bin/bash

 mongoimport --uri "mongodb://root:root@127.0.0.1:27017/?authSource=admin" \
  --db portfolio \
  --collection article \
  --file "article.json" --jsonArray 

 mongoimport --uri "mongodb://root:root@127.0.0.1:27017/?authSource=admin" \
  --db portfolio \
  --collection tag \
  --file "tag.json" --jsonArray 
