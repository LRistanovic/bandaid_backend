# BandAid
BandAid is an app intended to help musicians meet, form bands and schedule their practicing. It is an open source, side project consisting of multiple services. It's made primarily as a hobby and a learning experience.

# Backend

This repository holds the backend service for the app. It consists of a postgresql database and a RESTful server written in Rust (version 1.72.0) with the axum framework and diesel ORM. Those services can be run natively or in docker containers, in which case you can call
> docker compose up
in the main directory to run both of the servers.
