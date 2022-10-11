#!/bin/bash

set -e

gramine_version=1.2

repo=tdaas.tencentcloudcr.com/tdaas.dev/xiangminli/gramine-memory-overwrite-failure-demo 
tag=$(git rev-parse --short HEAD)-gramine${gramine_version}

repo_tag=$repo:$tag

docker build -t $repo_tag .

docker push $repo_tag
