const express = require('express');

const app = express()
const port = 8080


app.post('/', (req, res) => {
    console.log("hei");
})


app.listen(port, () => {
    console.log(`Server running on port ${port}`);
})