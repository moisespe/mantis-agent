#!/bin/bash

SERVICE_NAME=mantis-agente
BIN_PATH=/usr/local/bin/mantis-agente
VERSION=0.1.0

# descarga binario
sudo curl https://github.com/moisespe/mantis-agent/releases/download/$VERSION/mantis-agent -o /usr/local/bin/mantis-agent

# copiar binario
sudo cp mantis-agente $BIN_PATH
sudo chmod +x $BIN_PATH

# crear servicio
sudo tee /etc/systemd/system/$SERVICE_NAME.service > /dev/null <<EOF
[Unit]
Description=Mantis Agente de Monitoreo

[Service]
ExecStart=$BIN_PATH
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# recargar y activar
sudo systemctl daemon-reload
sudo systemctl enable $SERVICE_NAME
sudo systemctl start $SERVICE_NAME

echo "Servicio instalado y ejecutándose"