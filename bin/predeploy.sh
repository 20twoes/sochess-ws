# Build Docker image
# Make sure Docker daemon is running
docker build -t sochess .

# docker images
# docker save -o sochess-image.tar <container_id>
docker save -o sochess-image.tar sochess:latest
