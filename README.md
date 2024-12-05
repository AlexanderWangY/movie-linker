# FilmTopia
This project aims to emulate the idea that no two humans currently alive are separated by more than 6 degrees. In the same way, we hypthesize that no two films made after the 2000s have more than 6 degrees of connections between shared actors. That means any film in our dataset should be able to link to another based on actors within 6 nodes. Want to see if you can best our prediction? Try it out!

Steps to run:

You must have Cargo install and Rust installed. You can do this by going to Rust's [installation page](https://www.rust-lang.org/tools/install)

You must also have node installed as well! Download [here](https://nodejs.org/en/download/package-manager)


First go into `/server/src` and run the following:

`cargo build --release`
`../target/release/server`

### IMPORTANT: YOU MUST HAVE 16 GB of RAM

and then navigate to `/client` and run the following:

`npm install`
`npm run dev`

Then open the link provided in the terminal and try it out!
