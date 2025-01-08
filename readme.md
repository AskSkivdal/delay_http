# delay_http
A simple server to send a http request and for it to wait the specified amount of seconds until completion


## How to run
Requires docker
```
docker build -t delay_http . && docker run -it --rm -p 8080:8080 delay_http
```