# Prerequisites

## Build docker image
```bash
docker build -t softprops/lambda-rust:1.51 https://github.com/softprops/lambda-rust.git#e6137ddbac36d104236407eb537c6c03a16a30fa
```

# Deploy

```bash
yarn run serverless deploy [--aws-profile profile-name]
```

