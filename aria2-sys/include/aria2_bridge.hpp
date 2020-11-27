#pragma once
#include <string>
#include <memory>
#include <aria2/aria2.h>
#include "rust/cxx.h"
#include "aria2-sys/src/lib.rs.h"
#include "aria2-sys/include/DownloadHandleWrapper.hpp"

namespace aria2 {
    namespace bridge {
        using A2Gid = aria2::A2Gid;
        using RKeyVals = rust::Vec<KeyVal>;

        // Session creation

        SessionHandle sessionNew(
                const RKeyVals& rustOptions,
                const SessionConfigFfi& config,
                const rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> cb);

        int sessionFinal(SessionHandle session);

        // Run command

        int run(SessionHandle session, aria2::RUN_MODE runMode);
        int shutdown(SessionHandle session, bool force);

        // A2Gid utils

        rust::String gidToHex(A2Gid gid);
        A2Gid hexToGid(rust::Str hex);
        bool isGidNull(A2Gid gid);

        // Adds

        int addUri(SessionHandle session, A2Gid& gid, const rust::Vec<rust::String>& uris,
                    const RKeyVals& options, int position);

        int addMetalink(SessionHandle session, rust::Vec<A2Gid>& gids, const rust::Str metalinkFile,
                         const RKeyVals& options, int position);

        int addTorrent(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                        const RKeyVals& options, int position);

        int addTorrentWithWebseedUris(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                                          const rust::Vec<rust::String>& webSeedUris, const RKeyVals& options, int position);

        // Download control

        rust::Vec<A2Gid> getActiveDownload(SessionHandle session);
        int removeDownload(SessionHandle session, A2Gid gid, bool force);
        int pauseDownload(SessionHandle session, A2Gid gid, bool force);
        int unpauseDownload(SessionHandle session, A2Gid gid);
        int changePosition(SessionHandle session, A2Gid gid, int pos, aria2::OffsetMode how);

        // Options

        int changeOption(SessionHandle session, A2Gid gid, const RKeyVals& options);
        rust::Str getGlobalOption(SessionHandle session, const rust::Str name);
        RKeyVals getGlobalOptions(SessionHandle session);
        int changeGlobalOption(SessionHandle session, const RKeyVals& options);

        // Stats

        GlobalStat getGlobalStat(SessionHandle session);

        // Download Handle

        std::unique_ptr<DownloadHandleWrapper> getDownloadHandle(SessionHandle session, A2Gid gid);
        void deleteDownloadHandle(std::unique_ptr<DownloadHandleWrapper> handle);

        // Internals

        int __eventCallbackDelegate(Session* session, DownloadEvent event, A2Gid gid, void* userData);
        void __convertKeyVals(const RKeyVals& src, aria2::KeyVals& dst);
        void __convertKeyValsBack(const aria2::KeyVals& src, RKeyVals& dst);
    }
}
