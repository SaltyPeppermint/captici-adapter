FROM alpine

# Install thttpd
RUN apk add thttpd

# Create a non-root user to own the files and run our server
RUN adduser -D static
USER static
WORKDIR /static

# Copy the static website
# Use the .dockerignore file to control what ends up inside the image!
COPY target/release/test-adapter .

# Run thttpd
CMD ["thttpd", "-D", "-h", "0.0.0.0", "-p", "3000", "-d", "/static", "-u", "static", "-l", "-", "-M", "60"]
