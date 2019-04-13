## Rust Server

This program runs a server that is accessible on http://localhost:4000/.

When the server receives a request on http://localhost:4000/set?somekey=somevalue, it stores the passed key and value in memory.

When it receives a request on http://localhost:4000/get?key=somekey it returns the value stored at **somekey**.
