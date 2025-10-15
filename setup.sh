#!/bin/bash

# Exit on error
set -e

# Update system and install build dependencies
echo "Updating system and installing dependencies..."
sudo apt-get update -y
sudo apt-get upgrade -y
sudo apt-get install -y git cmake build-essential libgtk-3-dev libudev-dev libinput-dev libxkbcommon-dev

# Clone and build cyberdeck-os
echo "Cloning and building cyberdeck-os..."
cd /opt/cyberdeck-os
mkdir -p build
cd build
cmake ..
make

# Remove default desktop environment (e.g., GNOME, KDE, etc.)
echo "Removing default desktop environment..."
sudo apt-get purge -y ubuntu-desktop gnome-shell gdm3
sudo apt-get autoremove -y

# Create a systemd service for cyberdeck-os
echo "Creating systemd service for cyberdeck-os..."
sudo tee /etc/systemd/system/cyberdeck-os.service > /dev/null <<EOL
[Unit]
Description=Cyberdeck OS
After=network.target

[Service]
ExecStart=/opt/cyberdeck-os/build/main
Restart=always
User=$USER
Environment=DISPLAY=:0
Environment=XAUTHORITY=/home/$USER/.Xauthority

[Install]
WantedBy=multi-user.target
EOL

# Enable and start the service
echo "Enabling and starting cyberdeck-os service..."
sudo systemctl daemon-reload
sudo systemctl enable cyberdeck-os.service
sudo systemctl start cyberdeck-os.service

# Disable other display managers (e.g., gdm3, lightdm)
echo "Disabling other display managers..."
sudo systemctl disable gdm3 lightdm sddm

# Set up autologin and start cyberdeck-os as the only GUI
echo "Configuring autologin and GUI..."
sudo mkdir -p /etc/systemd/system/getty@tty1.service.d
sudo tee /etc/systemd/system/getty@tty1.service.d/override.conf > /dev/null <<EOL
[Service]
ExecStart=
ExecStart=-/sbin/agetty --autologin $USER --noclear %I \$TERM
EOL

# Start Xorg with cyberdeck-os
sudo tee /home/$USER/.bash_profile > /dev/null <<EOL
if [ -z "\$DISPLAY" ] && [ "\$(tty)" = "/dev/tty1" ]; then
    startx /opt/cyberdeck-os/build/main -- vt1
fi
EOL

echo "Setup complete. Reboot to start cyberdeck-os as the only GUI."
