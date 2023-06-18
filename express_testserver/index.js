const express = require('express');

const app = express()
const port = 8080



app.post('/', (req, res) => {
    console.log("hei");
    res.json({"hei": "hei"})
})

app.get('/', (req, res) => {
    console.log("hei2");
    res.json({"hei2": "hei2"})
})


app.listen(port, () => {
    console.log(`Server running on port ${port}`);
})