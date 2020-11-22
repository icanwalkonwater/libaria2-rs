#pragma once
#include <string>
#include <memory>
#include <aria2/aria2.h>
#include "rust/cxx.h"
#include "libaria2/src/lib.rs.h"

namespace aria2 {
    namespace bridge {
        using A2Gid = aria2::A2Gid;
        using RKeyVals = rust::Vec<KeyVal>;

        // Library init/destroy

        int library_init();
        int library_deinit();

        // Session creation

        SessionHandle session_new(
                const RKeyVals& rustOptions,
                const SessionConfigFfi& config,
                const rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> cb);

        int session_final(SessionHandle session);

        // Run command
        int run(SessionHandle session, aria2::RUN_MODE runMode);

        // A2Gid utils

        rust::String gid_to_hex(A2Gid gid);
        A2Gid hex_to_gid(rust::Str hex);
        bool is_gid_null(A2Gid gid);

        // Adds

        int add_uri(SessionHandle session, A2Gid& gid, const rust::Vec<rust::String>& uris,
                    const RKeyVals& options, int position);

        int add_metalink(SessionHandle session, rust::Vec<A2Gid>& gids, const rust::Str metalinkFile,
                         const RKeyVals& options, int position);

        int add_torrent(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                        const RKeyVals& options, int position);

        int add_torrent_with_webseed_uris(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                                          const rust::Vec<rust::String>& webSeedUris, const RKeyVals& options, int position);

        // Download control

        rust::Vec<A2Gid> get_active_download(SessionHandle session);
        int remove_download(SessionHandle session, A2Gid gid, bool force);
        int pause_download(SessionHandle session, A2Gid gid, bool force);
        int unpause_download(SessionHandle session, A2Gid gid);

        // Options

        int change_option(SessionHandle session, A2Gid gid, const RKeyVals& options);
        rust::Str get_global_option(SessionHandle session, const rust::Str name);
        RKeyVals get_global_options(SessionHandle session);
        int change_global_option(SessionHandle session, const RKeyVals& options);

        // Internals

        int __event_callback_delegate(Session* session, DownloadEvent event, A2Gid gid, void* userData);
        void __convert_key_vals(const RKeyVals& src, aria2::KeyVals& dst);
        void __convert_key_vals_back(const aria2::KeyVals& src, RKeyVals& dst);
    }
}
