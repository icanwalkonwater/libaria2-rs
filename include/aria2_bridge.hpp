#pragma once
#include <string>
#include <memory>
#include <aria2/aria2.h>
#include "rust/cxx.h"
#include "libaria2/src/lib.rs.h"

namespace aria2 {
    namespace bridge {
        using A2Gid = aria2::A2Gid;
        using DownloadEvent = aria2::DownloadEvent;

        int library_init();
        int library_deinit();

        SessionHandle session_new(const rust::Vec<KeyVal>& rustOptions, const SessionConfigFfi& config);
        int session_final(SessionHandle session);
    }
}
