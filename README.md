# Parker
Parker is a Rust-based templated website framework based on an idea by
[Joshua Driesman](https://github.com/JoshuaDriesman). Its main use is serving
server-side rendered HTML along with CSS and JS files.

## Development
### Running Locally
1. Clone the repo
2. Install docker and docker-compose
3. `docker-compose up`

### Adding Pages
The only way to add pages currently is through manually posting to the '/' endpoint. You must specify a route, the template to use (defaults to page), and the content as a file (currently just supports markdown). 

There is a script `post.py` to help with this. Usage is as follows

```python post.py [--url URL] [--template TEMPLATE] --content CONTENT route```

`TEMPLATE` is just the file name (without any extensions) of a template in the templates directory

`CONTENT` must be a file, currently only markdown is supported