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
                const rust::Vec<KeyVal>& rustOptions,
                const SessionConfigFfi& rustConfig,
                const rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> eventCallback
        ) {
            KeyVals options;
            for (auto opt : rustOptions) {
                options.push_back({std::string(opt.key), std::string(opt.val)});
            }

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

        // Internal
        int __event_callback_delegate(Session* session, DownloadEvent event, A2Gid gid, void* userData) {
            std::cout << "Callback" << std::endl;
            return EVENT_RUST_CALLBACK(
                    { .ptr = (size_t) session },
                    event,
                    gid,
                    (size_t) userData
            );
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
    }
}