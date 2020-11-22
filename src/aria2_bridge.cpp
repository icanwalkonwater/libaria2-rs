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

        int library_init() {
            return aria2::libraryInit();
        }

        int library_deinit() {
            return aria2::libraryDeinit();
        }

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

        int run(SessionHandle session, aria2::RUN_MODE runMode) {
            return aria2::run((Session*) session.ptr, runMode);
        }

        rust::String gid_to_hex(A2Gid gid) {
            return rust::String(aria2::gidToHex(gid));
        }

        A2Gid hex_to_gid(rust::Str hex) {
            return aria2::hexToGid(static_cast<std::string>(hex));
        }

        bool is_gid_null(A2Gid gid) {
            return aria2::isNull(gid);
        }

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

        // Internals

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
    }
}