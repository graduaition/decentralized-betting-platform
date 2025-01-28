const express = require('express');
const { redistributeGains } = require('./redistribute');
const { listenToTweets } = require('./twitterListener');

const app = express();
const PORT = 3000;

app.use(express.json());

app.post('/redistribute', async (req, res) => {
    const { winner } = req.body;
    await redistributeGains(winner);
    res.send('Redistribution done');
});

app.listen(PORT, () => {
    console.log(`Backend server listening the port ${PORT}`);
    listenToTweets();
});
