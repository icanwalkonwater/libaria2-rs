#pragma once
#include <string>
#include <memory>
#include <aria2/aria2.h>
#include "rust/cxx.h"
#include "libaria2/src/lib.rs.h"

namespace aria2 {
    namespace bridge {
        using A2Gid = aria2::A2Gid;

        int library_init();
        int library_deinit();

        SessionHandle session_new(
                const rust::Vec<KeyVal>& rustOptions,
                const SessionConfigFfi& config,
                const rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> cb);

        int session_final(SessionHandle session);

        // Internal
        int __event_callback_delegate(Session* session, DownloadEvent event, A2Gid gid, void* userData);

        int run(SessionHandle session, aria2::RUN_MODE runMode);

        rust::String gid_to_hex(A2Gid gid);
        A2Gid hex_to_gid(rust::Str hex);
        bool is_gid_null(A2Gid gid);
    }
}
