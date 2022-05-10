const { md5 } = require("./index.node");
const express = require('express')
const bodyParser = require('body-parser');
const cors = require('cors');
const fs = require('fs');
const app = express();
const port = 3001;
const path = require( "path" );
const fileUpload = require('express-fileupload');
app.use(fileUpload());
app.use(cors());
app.use(bodyParser.urlencoded({ extended: false }));
app.use(bodyParser.json());
app.get("/", function(req,res){
    res.send("Welcome to cloud");
});
app.post("/getfile", function(req,res){

  let filest = [];
  fs.readdirSync( req.body.folder ).forEach( files => {
        console.log("File: " + files)
        let absolutePath = path.resolve( req.body.folder, files );
        try {
            code = md5(absolutePath);
        } catch (error) {
            console.log("Not a file");
            code = "dir";
        }
        filest.push({
          file:absolutePath,
          md5:code
        })
 });
 res.send(filest)
});
app.use(express.urlencoded({ extended: true }));

app.listen(port, () => console.log(`CLURD API started on port ${port}!`));

