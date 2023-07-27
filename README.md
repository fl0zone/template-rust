<p align="center">
  <a href="https://fl0.com/" target="blank">
    <img src="https://user-images.githubusercontent.com/88681427/217122968-e6132cad-1944-4ebe-9ec1-105af6a18c4f.png">
  </a>
</p>

<h2 align="center">Rust + Axum + Serde Starter Pack</h2>
<p align="center">Backend engineering, supercharged.</p>

## Overview

Use this repository to get up and running on FL0 with the following stack:

<table>
<tr>
  <th>Language</th>
  <td>Rust</td>
</tr>
<tr>
  <th>Router</th>
  <td>Axum</td>
</tr>
<tr>
  <th>Serialization</th>
  <td>Serde</td>
</tr>
</table>

## Getting Started
We recommend using the provided Docker Compose configuration for local development. Our Docker configuration includes:
1. Automatically create app server easily
2. Production-ready minified image

However, you can still use this repo without Docker! See the instructions below.

### Using Docker
1. `docker compose up` (initial build can take a couple of mins)
2. That's it! Visit http://localhost:8080 to see your app running

### Without Docker
If you don't want to use Docker, run your rust app as per usual
1. `cargo run`
2. Visit http://localhost:3000 to see your app running

Note: you can optionally set the `PORT` env var to see your app running on a port other than the default `3000`.

## Deploying to FL0
Checkout our [Getting Started Guide](https://docs.fl0.com) in the FL0 documentation!

## Questions
If you have any questions about FL0 or this template codebase please head on over to our [Discord channel](https://discord.gg/AmmVTt9Jrw).

## Issues
Any issues or feature requests can be raised on the [Issues page](https://github.com/fl0zone/template-rust) of this repo.

## License
This template repository is [MIT licensed](LICENSE).
