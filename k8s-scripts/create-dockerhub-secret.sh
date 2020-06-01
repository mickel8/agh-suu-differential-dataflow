#!/bin/bash
CONFIG_FILENAME=config.json
CONFIG_DIR=.docker
CONFIG_PATH=$CONFIG_DIR/$CONFIG_FILENAME

echo -n 'Docker Hub username: '
read DOCKER_HUB_REPOSITORY_USERNAME
echo -n 'Docker Hub password: '
read -s DOCKER_HUB_REPOSITORY_PASSWORD
echo ''

mkdir -p $CONFIG_DIR
DOCKERHUB_AUTH="$(echo -n $DOCKER_HUB_REPOSITORY_USERNAME:$DOCKER_HUB_REPOSITORY_PASSWORD | base64)"
echo "{\"auths\":{\"https://index.docker.io/v1/\":{\"auth\":\"${DOCKERHUB_AUTH}\"}}}" > $CONFIG_PATH

kubectl create secret generic regcred \
    --from-file=.dockerconfigjson=$CONFIG_PATH \
    --type=kubernetes.io/dockerconfigjson

rm $CONFIG_PATH
rmdir $CONFIG_DIR