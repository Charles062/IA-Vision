#!/bin/bash
# Install system dependencies for OpenVy-Win on Arch/Garuda Linux

# Update system
sudo pacman -Syu

# Install Tesseract and data files
sudo pacman -S --needed tesseract tesseract-data-eng tesseract-data-por

# Install build dependencies (clang, pkg-config, etc.)
sudo pacman -S --needed base-devel clang

# Install DBus and accessibility libraries (usually present, but ensuring)
sudo pacman -S --needed dbus at-spi2-core

echo "Environment setup complete."
