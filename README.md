
# QR Code Generator

A simple full-stack QR Code Generator built with **Rust (Axum)** on the backend and **Vue.js** on the frontend.

> ⚠️ This project is no longer actively maintained. It was originally created as a personal learning exercise to explore Rust and Vue.js.

---

## 🚧 Project Status

This was one of my first full-stack projects and is **not production-ready**. While many core features work as intended, the codebase has several beginner-level design issues that would need to be improved in a future rewrite.

---

## ✨ Features

- ✅ Generate QR codes with:
  - Custom colors
  - Optional logo/image overlays
  - Optional background patterns
- ✅ User registration and login
- ✅ Save and manage generated QR codes per user
- ✅ Edit existing QR codes
- ✅ Toggle between light and dark mode
- ✅ Download QR codes as SVG

---

## ❌ Known Issues / Missing Features

- ❌ **Authentication/session handling is not secure**
  - Sessions are not implemented safely for non-localhost deployments.
- ❌ **Error handling is minimal**
  - Many operations do not return meaningful error messages.
- ❌ **No tests written**
  - Neither unit nor integration tests exist.
- ❌ **Image/logo overlay is too small**
  - Logos placed on the QR codes are not scaled well.
- ❌ **Code structure needs heavy refactoring**
  - Variable/function naming, file organization, and logic separation are not well thought-out.

---

## 🛠 Planned (but not implemented)

- Refactoring project layout and improving maintainability
- Rewriting authentication with JWT or secure sessions
- Adding proper error responses and logging
- Writing tests
- Better frontend UX (image handling, previews, QR customization)

---

## 🧪 Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/) (make sure the database server is running)

---

## ⚙️ Setup Instructions

1. **Clone the repository**

   ```bash
   git clone https://github.com/Nerfned/qrcode-generator.git
   cd qrcode-generator
   ```

2. **Edit the configuration file**

   Update `config.toml` to match your local PostgreSQL credentials and desired server port.

   Example:
   ```toml
   port = 3000
   dir = "frontend/dist"
   file = "index.html"

   username = "postgres"
   password = "yourpassword"
   host = "localhost"
   database = "qrcode"
   ```

3. **Run the backend**

   ```bash
   cargo run
   ```

4. The server should now be available at:

   ```
   http://localhost:<PORT>
   ```

   Replace `<PORT>` with the value from your `config.toml`.

---

## 🧩 File Structure Notes

- Backend is written in Rust using `axum` and `sqlx`.
- Frontend is located in the `frontend` directory using Vue.js + Vite.
- Assets such as logos, buttons, and styles are under `src/assets`.

---

## 📌 Final Words

This project was built as a learning exercise in Rust and Vue.js. While it is functional in many ways, it reflects a beginner-level understanding at the time and has **significant room for improvement**. However, it also holds **great potential** if extended with the knowledge I’ve gained since then.
