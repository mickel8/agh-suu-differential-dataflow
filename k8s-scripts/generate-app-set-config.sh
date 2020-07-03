#!/bin/bash
FILE=app-set
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
echo -n 'imagePullPolicy: ([A]lways/[n]ever)? '
read -n 1 -r IMAGE_PULL_POLICY
echo
echo -n 'Enter cluster size: '
read CLUSTER_SIZE
echo -n 'Enter number of workers per node: '
read WORKERS_PER_NODE
echo

if [[ $IMAGE_PULL_POLICY =~ ^[Aa]$ ]]
then
  IMAGE_PULL_POLICY='Always'
else
  IMAGE_PULL_POLICY='Never'
fi

sed -e "s/\${DOCKERHUB_ID}/$DOCKERHUB_ID/" \
  -e "s/\${DOCKERHUB_REPO}/$DOCKERHUB_REPO/" \
  -e "s/\${DOCKERHUB_REPO_TAG}/$DOCKERHUB_REPO_TAG/" \
  -e "s/\${IMAGE_PULL_POLICY}/$IMAGE_PULL_POLICY/" \
  -e "s/\${CLUSTER_SIZE}/$CLUSTER_SIZE/" \
  -e "s/\${WORKERS_PER_NODE}/$WORKERS_PER_NODE/" \
  $SCHEME_PATH > $FILE_PATH

if [[ $USE_SECRET =~ ^[Nn]$ ]]
then
  sed -i '58,59d' $FILE_PATH
fi
