# QR Code Generator

This project was originally created as a way to learn Rust and Vue.js. It started as a personal learning project and was never intended to be a finished or production-ready application.

The project has since been paused. There are many things that would need to be improved or restructured—both in the code and in how everything is organized.

Some examples:
- Variable and function names are often not very meaningful.
- The structure of the project needs a lot of work. It's not always clear where to find things.
- The login and session system is not secure enough for anything beyond local use.
- Error handling is almost completely missing.
- No tests have been written.
- The image embedding feature works, but the images are too small on the QR code and would need adjustment.

That said, a few core features already work reliably.

## Currently working features

- Generate QR codes with custom colors, patterns, and optional embedded images
- Download generated QR codes
- User registration, login, and profile editing
- Save and edit previously created QR codes
- Light and dark mode in the frontend

## Requirements

To run the QR Code Generator, you’ll need the following installed:

- Rust
- PostgreSQL

Node.js is only required if you want to make changes to the frontend. If you just want to run the app as-is, it's not necessary.

## Setup

1. Install Rust if you don’t have it already: https://www.rust-lang.org/tools/install

2. Install PostgreSQL and make sure you have a user that can access the database.

3. Change the `config.toml` file in the project root with your Information.

4. Start the backend:

   `cargo run`

5. You can now access the application at:

   `http://localhost:8000`

## Notes

This is clearly a beginner project. Since building it, I’ve learned a lot and would approach many parts of it differently now. It’s not perfect, but it’s a solid starting point for further development or learning.
