#!/bin/bash
FILE=kaniko-pod
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

sed -e "s/\${DOCKERHUB_ID}/$DOCKERHUB_ID/" \
  -e "s/\${DOCKERHUB_REPO}/$DOCKERHUB_REPO/" \
  -e "s/\${DOCKERHUB_REPO_TAG}/$DOCKERHUB_REPO_TAG/" \
  $SCHEME_PATH > $FILE_PATH
