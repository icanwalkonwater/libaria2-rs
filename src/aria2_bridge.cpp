#include <aria2/aria2.h>
#include <vector>
#include <memory>
#include <iterator>
#include <algorithm>
#include <functional>
#include <iostream>
#include "rust/cxx.h"
#include "libaria2/include/aria2_bridge.hpp"
#include "libaria2/src/lib.rs.h"

namespace aria2 {
    namespace bridge {
        static rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> EVENT_RUST_CALLBACK;

        // Library init/deinit
        // <editor-fold>

        int library_init() {
            return aria2::libraryInit();
        }

        int library_deinit() {
            return aria2::libraryDeinit();
        }
        // </editor-fold>

        // Session
        // <editor-fold>

        SessionHandle session_new(
                const RKeyVals& rustOptions,
                const SessionConfigFfi& rustConfig,
                const rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> eventCallback
        ) {
            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            SessionConfig config;
            config.keepRunning = rustConfig.keep_running;
            config.useSignalHandler = rustConfig.use_signal_handler;
            config.downloadEventCallback = &aria2::bridge::__event_callback_delegate;
            config.userData = (void*) rustConfig.user_data;

            EVENT_RUST_CALLBACK = eventCallback;

            return { .ptr = (size_t) sessionNew(options, config) };
        }

        int session_final(SessionHandle session) {
            return aria2::sessionFinal((Session*) session.ptr);
        }

        // </editor-fold>

        // Run

        int run(SessionHandle session, aria2::RUN_MODE runMode) {
            return aria2::run((Session*) session.ptr, runMode);
        }

        int shutdown(SessionHandle session, bool force) {
            return aria2::shutdown((Session*) session.ptr, force);
        }

        // Utils
        // <editor-fold>

        rust::String gid_to_hex(A2Gid gid) {
            return rust::String(aria2::gidToHex(gid));
        }

        A2Gid hex_to_gid(rust::Str hex) {
            return aria2::hexToGid(static_cast<std::string>(hex));
        }

        bool is_gid_null(A2Gid gid) {
            return aria2::isNull(gid);
        }
        // </editor-fold>

        // Adds
        // <editor-fold>

        int add_uri(SessionHandle session, A2Gid& gid, const rust::Vec<rust::String>& rustUris,
                    const RKeyVals& rustOptions, int position) {
            std::vector<std::string> uris;
            uris.resize(rustUris.size());
            for (auto uri : rustUris) {
                uris.emplace_back(uri);
            }

            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            return aria2::addUri((Session*) session.ptr, &gid, uris, options, position);
        }

        int add_metalink(SessionHandle session, rust::Vec<A2Gid>& gids, const rust::Str metalinkFile,
                         const RKeyVals& rustOptions, int position) {
            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            std::vector<A2Gid> resGuids;
            int res = aria2::addMetalink((Session*) session.ptr, &resGuids, std::string(metalinkFile), options, position);
            for (auto guid : resGuids) {
                gids.push_back(guid);
            }

            return res;
        }

        int add_torrent(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                        const RKeyVals& rustOptions, int position) {
            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            return aria2::addTorrent((Session*) session.ptr, &gid, std::string(torrentFile), options, position);
        }

        int add_torrent_with_webseed_uris(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                                          const rust::Vec<rust::String>& rustWebSeedUris, const RKeyVals& rustOptions, int position) {
            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            std::vector<std::string> webSeedUris;
            webSeedUris.reserve(rustWebSeedUris.size());
            for (auto uri : rustWebSeedUris) {
                webSeedUris.emplace_back(uri);
            }

            return aria2::addTorrent((Session*) session.ptr, &gid, std::string(torrentFile), webSeedUris, options, position);
        }
        // </editor-fold>

        // Download control
        // <editor-fold>

        rust::Vec<A2Gid> get_active_download(SessionHandle session) {
            std::vector<A2Gid> res = aria2::getActiveDownload((Session*) session.ptr);
            rust::Vec<A2Gid> rustRes;
            std::copy(res.begin(), res.end(), std::back_inserter(rustRes));

            return rustRes;
        }

        int remove_download(SessionHandle session, A2Gid gid, bool force) {
            return aria2::removeDownload((Session*) session.ptr, gid, force);
        }

        int pause_download(SessionHandle session, A2Gid gid, bool force) {
            return aria2::pauseDownload((Session*) session.ptr, gid, force);
        }

        int unpause_download(SessionHandle session, A2Gid gid) {
            return aria2::unpauseDownload((Session*) session.ptr, gid);
        }

        int change_position(SessionHandle session, A2Gid gid, int pos, aria2::OffsetMode how) {
            return aria2::changePosition((Session*) session.ptr, gid, pos, how);
        }

        // </editor-fold>

        // Options
        // <editor-fold>

        int change_option(SessionHandle session, A2Gid gid, const RKeyVals& rustOptions) {
            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            return aria2::changeOption((Session*) session.ptr, gid, options);
        }

        rust::Str get_global_option(SessionHandle session, const rust::Str name) {
            return rust::Str(aria2::getGlobalOption((Session*) session.ptr, std::string(name)));
        }

        RKeyVals get_global_options(SessionHandle session) {
            aria2::KeyVals options(aria2::getGlobalOptions((Session*) session.ptr));
            RKeyVals rustOptions;
            __convert_key_vals_back(options, rustOptions);

            return rustOptions;
        }

        int change_global_option(SessionHandle session, const RKeyVals& rustOptions) {
            aria2::KeyVals options;
            __convert_key_vals(rustOptions, options);

            return aria2::changeGlobalOption((Session*) session.ptr, options);
        }
        // </editor-fold>

        // Stats

        GlobalStat get_global_stat(SessionHandle session) {
            aria2::GlobalStat stat = aria2::getGlobalStat((Session*) session.ptr);
            return {
                .download_speed = stat.downloadSpeed,
                .upload_speed = stat.uploadSpeed,
                .num_active = stat.numActive,
                .num_waiting = stat.numWaiting,
                .num_stopped = stat.numStopped
            };
        }

        // Download Handle
        // <editor-fold>

        rust::String DownloadHandle_getBitfieldExt(aria2::DownloadHandle& handle) {
            return rust::String(handle.getBitfield());
        }

        std::unique_ptr<std::vector<aria2::FileData>> DownloadHandle_getFiles(aria2::DownloadHandle& handle) {
            std::vector<aria2::FileData> original(handle.getFiles());
            /*rust::Vec<aria2::FileData> out;
            std::copy(original.begin(), original.end(), std::back_inserter(original));
            return out;*/
            return std::make_unique<std::vector<aria2::FileData>>(original);
        }

        std::unique_ptr<aria2::FileData> DownloadHandle_getFile(aria2::DownloadHandle& handle, int index) {
            std::cout << "hey" << std::endl;
            aria2::FileData data = handle.getFile(index);
            std::cout << "ho" << std::endl;
            return std::make_unique<aria2::FileData>(data);
        }

        std::unique_ptr<aria2::BtMetaInfoData> DownloadHandle_getBtMetaInfo(aria2::DownloadHandle& handle) {
            aria2::BtMetaInfoData data(handle.getBtMetaInfo());
            return std::make_unique<aria2::BtMetaInfoData>(data);
        }

        RKeyVals DownloadHandle_getOptions(aria2::DownloadHandle& handle) {
            aria2::KeyVals original(handle.getOptions());
            RKeyVals out;
            __convert_key_vals_back(original, out);
            return out;
        }

        std::unique_ptr<aria2::DownloadHandle> get_download_handle(SessionHandle session, A2Gid gid) {
            aria2::DownloadHandle* handle = aria2::getDownloadHandle((Session*) session.ptr, gid);
            return std::unique_ptr<aria2::DownloadHandle>(handle);
        }

        void delete_download_handle(std::unique_ptr<aria2::DownloadHandle> handle) {
            aria2::deleteDownloadHandle(handle.get());
        }

        // </editor-fold>

        // Internals
        // <editor-fold>

        int __event_callback_delegate(Session* session, DownloadEvent event, A2Gid gid, void* userData) {
            return EVENT_RUST_CALLBACK(
                    { .ptr = (size_t) session },
                    event,
                    gid,
                    (size_t) userData
            );
        }

        void __convert_key_vals(const RKeyVals& src, aria2::KeyVals& dst) {
            dst.reserve(src.size());
            for (auto item : src) {
                dst.push_back({ std::string(item.key), std::string(item.val) });
            }
        }

        void __convert_key_vals_back(const aria2::KeyVals& src, RKeyVals& dst) {
            dst.reserve(src.size());
            for (auto item : src) {
                dst.push_back({
                    .key = rust::String(item.first),
                    .val = rust::String(item.second),
                });
            }
        }

        // </editor-fold>
    }
}