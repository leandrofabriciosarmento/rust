# Use an official Rust runtime as a parent image
FROM rust:1.54

# Set the working directory in the container to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Install any needed packages specified in Cargo.toml
RUN cargo install --path .

# Make port 8080 available to the world outside this container
EXPOSE 8080

# Define environment variable
ENV NAME World

# Run the binary program produced by `cargo install`
CMD ["rust-rest-api"]