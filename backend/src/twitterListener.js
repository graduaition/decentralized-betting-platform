const Twitter = require('twitter-lite');
const { takeSnapshot } = require('./solanaHandler');

const client = new Twitter({
    consumer_key: 'KEY',
    consumer_secret: 'SECRET',
    access_token_key: 'TOKEN',
    access_token_secret: 'SECRET_TOKEN',
});

function listenToTweets() {
    const stream = client.stream('statuses/filter', { followed: ['ID_MACHIPROFESSOR'] });
    stream.on('data', (tweet) => {
        console.log('Nouveau tweet:', tweet.text);
        takeSnapshot();
    });
    stream.on('error', (error) => {
        console.error('Erreur:', error);
    });
}

module.exports = { listenToTweets };
