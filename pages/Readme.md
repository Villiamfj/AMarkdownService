# A markdown service

This is a service made to host markdown files on the web using server-side rendering.

### These are the crates used:
- pulldown-cmark is used to parse markdown to HTML
- Yew is used to render pages.
- Rocket is used to serve pages.

# How to run
1. Put markdown files into the Pages folder
2. Edit, replace or keep the main.css file
3. `cargo run --release`

## How to run it with docker
1. `docker build -t <INSERT NAME HERE>`
2. `docker run -p 8000:8000 <INSERT NAME HERE>`