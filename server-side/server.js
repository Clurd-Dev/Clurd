const express = require('express')
const bodyParser = require('body-parser');
const cors = require('cors');
const fs = require('fs');
const app = express();
const port = 3001;
const path = require('path');
const { exec } = require("child_process");
const { spawn } = require("child_process");
const fileUpload = require('express-fileupload');
const { stdout } = require('process');
const execSync = require('child_process').execSync;

app.use(fileUpload());
app.use(cors());
app.use(bodyParser.urlencoded({ extended: false }));
app.use(bodyParser.json());
app.get("/", function(req,res){
    res.send("Welcome to cloud");
});
app.post("/getfile", function(req,res){

  let files = [];
  fs.readdir(req.body.folder, (err, filesraw) => {
      filesraw.forEach(file => {
        code = execSync('./md5 ' + file, {encoding: 'utf8'});
        files.push({
          file:file,
          md5:code
        })
    });
    res.send(files)
});

});
app.use(express.urlencoded({ extended: true }));

app.listen(port, () => console.log(`CLOUD API started on port ${port}!`));
