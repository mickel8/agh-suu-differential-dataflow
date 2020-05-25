#!/bin/bash
FILE=app-deployment
SCHEME=$FILE-scheme
DIR=k8s-config
SCHEME_PATH=$DIR/$SCHEME
FILE_PATH=$DIR/$FILE.yaml

echo -n 'Enter Docker Hub ID: '
read DOCKERHUB_ID
echo -n 'Enter Repository name: '
read DOCKERHUB_REPO
echo -n 'Enter Repository tag: '
read DOCKERHUB_REPO_TAG
echo -n 'Use secret (y/n)? '
read -n 1 -r USE_SECRET
echo

sed -e "s/\${DOCKERHUB_ID}/$DOCKERHUB_ID/" \
  -e "s/\${DOCKERHUB_REPO}/$DOCKERHUB_REPO/" \
  -e "s/\${DOCKERHUB_REPO_TAG}/$DOCKERHUB_REPO_TAG/" \
  $SCHEME_PATH > $FILE_PATH

if [[ $USE_SECRET =~ ^[Nn]$ ]]
then
  sed -i '29,30d' $FILE_PATH
fi
