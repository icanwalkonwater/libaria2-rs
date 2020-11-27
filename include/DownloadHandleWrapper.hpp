#pragma once

#include <iostream>
#include <stdint.h>
#include <assert.h>
#include <aria2/aria2.h>
#include "rust/cxx.h"
#include "libaria2/src/lib.rs.h"
#include "libaria2/include/aria2_bridge.hpp"

namespace aria2 {
    namespace bridge {
        struct KeyVal;

        struct UriDataWrapper {
        public:
            explicit inline UriDataWrapper(aria2::UriData data) : uriData(std::move(data)) {
            }

            inline const std::string& getUri() const {
                return uriData.uri;
            }

            inline aria2::UriStatus getStatus() const {
                return uriData.status;
            }

        private:
            aria2::UriData uriData;
        };

        struct FileDataWrapper {
        public:
            inline explicit FileDataWrapper(aria2::FileData data) : fileData(std::move(data)) {
                for (const auto& uri : fileData.uris) {
                    uris.emplace_back(uri);
                }
            }

            inline unsigned int getIndex() const {
                return static_cast<unsigned int>(fileData.index);
            }

            inline const std::string& getPath() const {
                return fileData.path;
            }

            inline uint64_t getLength() const {
                return static_cast<uint64_t>(fileData.length);
            }

            inline uint64_t getCompletedLength() const {
                return static_cast<uint64_t>(fileData.completedLength);
            }

            inline bool isSelected() const {
                return fileData.selected;
            }

            inline const std::vector<UriDataWrapper>& getUris() const {
                return uris;
            }

        private:
            aria2::FileData fileData;
            std::vector<UriDataWrapper> uris;
        };

        struct DownloadHandleWrapper {
        public:
            inline explicit DownloadHandleWrapper(aria2::DownloadHandle* handle) : handle(handle) {
            }

            inline ~DownloadHandleWrapper() {
                aria2::deleteDownloadHandle(handle);
            }

            inline aria2::DownloadStatus getStatus() const {
                return handle->getStatus();
            }

            inline size_t getTotalLength() const {
                return static_cast<size_t>(handle->getTotalLength());
            }

            inline size_t getCompletedLength() const {
                return static_cast<size_t>(handle->getCompletedLength());
            }

            inline size_t getUploadLength() const {
                return static_cast<size_t>(handle->getUploadLength());
            }

            inline rust::String getBitfield() const {
                // We can't pass std::string by value, we need to make a copy of some sort.
                return rust::String(handle->getBitfield());
            }

            inline unsigned int getDownloadSpeed() const {
                return static_cast<unsigned int>(handle->getDownloadSpeed());
            }

            inline unsigned int getUploadSpeed() const {
                return static_cast<unsigned int>(handle->getUploadSpeed());
            }

            inline const std::string& getInfoHash() const {
                return handle->getInfoHash();
            }

            inline size_t getPieceLength() const {
                return handle->getPieceLength();
            }

            inline unsigned int getNumPieces() const {
                return static_cast<unsigned int>(handle->getNumPieces());
            }

            inline unsigned int getConnections() const {
                return static_cast<unsigned int>(handle->getConnections());
            }

            inline int getErrorCode() const {
                return handle->getErrorCode();
            }

            inline const std::vector<aria2::A2Gid>& getFollowedBy() const {
                return handle->getFollowedBy();
            }

            inline aria2::A2Gid getFollowing() const {
                return handle->getFollowing();
            }

            inline aria2::A2Gid getBelongsTo() const {
                return handle->getBelongsTo();
            }

            inline const std::string& getDir() const {
                return handle->getDir();
            }

            inline std::unique_ptr<std::vector<FileDataWrapper>> getFiles() const {
                std::unique_ptr<std::vector<FileDataWrapper>> vec = std::make_unique<std::vector<FileDataWrapper>>();
                vec->reserve(handle->getNumFiles());
                for (const auto& file : handle->getFiles()) {
                    vec->emplace_back(file);
                }
                return vec;
            }

            inline unsigned int getNumFiles() const {
                return static_cast<unsigned int>(handle->getNumFiles());
            }

            inline std::unique_ptr<FileDataWrapper> getFile(unsigned int index) const {
                assert(index >= 1);
                aria2::FileData file = handle->getFile((int) index);
                auto filePtr = std::make_unique<FileDataWrapper>(file);
                return filePtr;
            }

            inline std::unique_ptr<aria2::BtMetaInfoData> getBtMetaInfo() const {
                return std::make_unique<aria2::BtMetaInfoData>(handle->getBtMetaInfo());
            }

            inline const std::string& getOption(rust::Str name) const {
                std::string key(name);
                return handle->getOption(key);
            }

            rust::Vec<KeyVal> getOptions() const;

        private:
            aria2::DownloadHandle* handle;
        };
    }
}
