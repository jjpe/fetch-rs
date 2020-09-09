const fetch = require('node-fetch');

exports.fetch = fetch;
exports.Body = fetch.Body;
exports.Response = fetch.Response;


exports.fetch_json = function(url, options) {
    return fetch(url, options).then(result => result.json());
};
exports.fetch_text = function(url, options) {
    return fetch(url, options).then(result => result.text());
};
