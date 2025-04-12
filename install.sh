#!/bin/bash

set -e  # Exit on any error
set -u  # Treat unset variables as errors

# COLORS
GREEN="\033[0;32m"
RED="\033[0;31m"
NC="\033[0m"

echo -e "${GREEN}Starting installation script...${NC}"

# Update system
echo -e "${GREEN}Updating system packages...${NC}"
sudo apt update && sudo apt upgrade -y

# Install dependencies
echo -e "${GREEN}Installing dependencies: git, make, curl, ca-certificates, gnupg, lsb-release...${NC}"
sudo apt install -y git make curl ca-certificates gnupg lsb-release

# Install NGINX
echo -e "${GREEN}Installing NGINX...${NC}"
sudo apt install -y nginx
sudo systemctl enable nginx
sudo systemctl start nginx

# Install Docker
if ! command -v docker &> /dev/null
then
    echo -e "${GREEN}Installing Docker Engine...${NC}"
    # Add Docker's official GPG key
    sudo install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/$(. /etc/os-release && echo "$ID")/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    sudo chmod a+r /etc/apt/keyrings/docker.gpg

    # Set up the repository
    echo \
      "deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/$(. /etc/os-release && echo "$ID") \
      $(lsb_release -cs) stable" | \
      sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

    # Install Docker packages
    sudo apt update
    sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

    # Enable Docker service
    sudo systemctl enable docker
    sudo systemctl start docker

    # Add current user to docker group
    sudo usermod -aG docker "$USER"

    echo -e "${GREEN}Docker installed. Please log out and log back in to use Docker without sudo.${NC}"
else
    echo -e "${GREEN}Docker is already installed.${NC}"
fi

# Check Docker Compose v2
if docker compose version &> /dev/null
then
    echo -e "${GREEN}Docker Compose v2 is ready to use via 'docker compose'.${NC}"
else
    echo -e "${RED}Docker Compose v2 not found. Something went wrong.${NC}"
    exit 1
fi

echo -e "${GREEN}All tools installed successfully!${NC}"
