const { md5 } = require("./index.node");
const express = require('express')
const bodyParser = require('body-parser');
const cors = require('cors');
const fs = require('fs');
const app = express();
const port = 3001;
const fileUpload = require('express-fileupload');
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
        code = md5(file);
        files.push({
          file:file,
          md5:code
        })
    });
    res.send(files)
});

});
app.use(express.urlencoded({ extended: true }));

app.listen(port, () => console.log(`CLURD API started on port ${port}!`));

