// include/blobstore.h

#pragma once
#include <memory>
#include <string>

class BlobstoreClient {
public:
  BlobstoreClient();
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();