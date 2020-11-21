#include <aria2/aria2.h>
#include <vector>
#include <memory>
#include <iterator>
#include <algorithm>
#include <iostream>
#include "rust/cxx.h"
#include "libaria2/include/aria2_bridge.hpp"
#include "libaria2/src/lib.rs.h"

namespace aria2 {
    namespace bridge {
        int library_init() {
            return aria2::libraryInit();
        }

        int library_deinit() {
            return aria2::libraryDeinit();
        }

        SessionHandle session_new(const rust::Vec<KeyVal>& rustOptions, const SessionConfigFfi& rustConfig) {
            KeyVals options;
            for (auto opt : rustOptions) {
                options.push_back({std::string(opt.key), std::string(opt.val)});
            }

            SessionConfig config;
            config.keepRunning = rustConfig.keep_running;
            config.useSignalHandler = rustConfig.use_signal_handler;
            config.downloadEventCallback = nullptr;
            config.userData = (void*) rustConfig.user_data;

            return { .ptr = (size_t) sessionNew(options, config) };
        }

        int session_final(SessionHandle session) {
            return aria2::sessionFinal((Session*) session.ptr);
        }
    }
}