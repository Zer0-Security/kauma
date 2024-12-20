# Name for the temporary container
$CONTAINER_NAME = "rust-ci"

docker pull ghcr.io/johndoe31415/labwork-docker:master
docker tag ghcr.io/johndoe31415/labwork-docker:master labwork
# Start the Docker container in detached mode
Write-Host "Starting the Docker container..."
docker run --network none -d -it --name $CONTAINER_NAME labwork sleep infinity

# Check if the container started correctly
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to start Docker container."
    exit 1
}

# Copy all the files from the current directory into /home inside the container
Write-Host "Copying files to container..."

docker exec $CONTAINER_NAME mkdir -p "/r22/src"
docker cp "./src" "${CONTAINER_NAME}:/r22"
docker cp "./build" "${CONTAINER_NAME}:/r22"
docker cp "./kauma" "${CONTAINER_NAME}:/r22"
docker cp "./Cargo.toml" "${CONTAINER_NAME}:/r22"

# Check if the copy operation succeeded
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to copy files to the container."
    docker stop $CONTAINER_NAME
    docker rm $CONTAINER_NAME
    exit 1
}

# Run the build script inside the container
Write-Host "Running the build script..."
docker exec -w /r22 $CONTAINER_NAME ./build

# Check if the build script succeeded
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed."
    docker stop $CONTAINER_NAME
    docker rm $CONTAINER_NAME
    exit 1
}

# Run the tests inside the container and capture the output
Write-Host "Running tests in the container..."
docker exec -w /r22 $CONTAINER_NAME cargo test

# Capture the result of the tests
$test_result = $LASTEXITCODE

# Stop and remove the container
Write-Host "Cleaning up..."
docker stop $CONTAINER_NAME | Out-Null
docker rm $CONTAINER_NAME | Out-Null

# Check the test result and print a message
if ($test_result -ne 0) {
    Write-Host "Tests failed."
    exit 1
} else {
    Write-Host "All tests passed!"
    exit 0
}
