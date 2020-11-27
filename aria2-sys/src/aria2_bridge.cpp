#include <aria2/aria2.h>
#include <vector>
#include <memory>
#include <algorithm>
#include <functional>
#include "rust/cxx.h"
#include "aria2-sys/include/aria2_bridge.hpp"
#include "aria2-sys/src/lib.rs.h"

namespace aria2 {
    namespace bridge {
        static rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)> EVENT_RUST_CALLBACK;

        // Session
        // <editor-fold>

        SessionHandle sessionNew(
                const RKeyVals& rustOptions,
                const SessionConfigFfi& rustConfig,
                const rust::Fn<int(SessionHandle s, DownloadEvent e, A2Gid g, size_t user)>& eventCallback
        ) {
            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            SessionConfig config;
            config.keepRunning = rustConfig.keep_running;
            config.useSignalHandler = rustConfig.use_signal_handler;
            config.downloadEventCallback = &aria2::bridge::__eventCallbackDelegate;
            config.userData = (void*) rustConfig.user_data;

            EVENT_RUST_CALLBACK = eventCallback;

            return {.ptr = (size_t) sessionNew(options, config)};
        }

        int sessionFinal(SessionHandle session) {
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

        rust::String gidToHex(A2Gid gid) {
            return rust::String(aria2::gidToHex(gid));
        }

        A2Gid hexToGid(rust::Str hex) {
            return aria2::hexToGid(static_cast<std::string>(hex));
        }

        bool isGidNull(A2Gid gid) {
            return aria2::isNull(gid);
        }
        // </editor-fold>

        // Adds
        // <editor-fold>

        int addUri(SessionHandle session, A2Gid& gid, const rust::Vec<rust::String>& rustUris,
                   const RKeyVals& rustOptions, int position) {
            std::vector<std::string> uris;
            uris.resize(rustUris.size());
            for (auto uri : rustUris) {
                uris.emplace_back(uri);
            }

            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            return aria2::addUri((Session*) session.ptr, &gid, uris, options, position);
        }

        int addMetalink(SessionHandle session, rust::Vec<A2Gid>& gids, const rust::Str metalinkFile,
                        const RKeyVals& rustOptions, int position) {
            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            std::vector<A2Gid> resGuids;
            int res = aria2::addMetalink(
                    (Session*) session.ptr, &resGuids, std::string(metalinkFile), options, position
            );
            for (auto guid : resGuids) {
                gids.push_back(guid);
            }

            return res;
        }

        int addTorrent(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                       const RKeyVals& rustOptions, int position) {
            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            return aria2::addTorrent((Session*) session.ptr, &gid, std::string(torrentFile), options, position);
        }

        int addTorrentWithWebseedUris(SessionHandle session, A2Gid& gid, const rust::Str torrentFile,
                                      const rust::Vec<rust::String>& rustWebSeedUris, const RKeyVals& rustOptions,
                                      int position) {
            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            std::vector<std::string> webSeedUris;
            webSeedUris.reserve(rustWebSeedUris.size());
            for (auto uri : rustWebSeedUris) {
                webSeedUris.emplace_back(uri);
            }

            return aria2::addTorrent(
                    (Session*) session.ptr, &gid, std::string(torrentFile), webSeedUris, options, position
            );
        }
        // </editor-fold>

        // Download control
        // <editor-fold>

        rust::Vec<A2Gid> getActiveDownload(SessionHandle session) {
            std::vector<A2Gid> res = aria2::getActiveDownload((Session*) session.ptr);
            rust::Vec<A2Gid> rustRes;
            std::copy(res.begin(), res.end(), std::back_inserter(rustRes));

            return rustRes;
        }

        int removeDownload(SessionHandle session, A2Gid gid, bool force) {
            return aria2::removeDownload((Session*) session.ptr, gid, force);
        }

        int pauseDownload(SessionHandle session, A2Gid gid, bool force) {
            return aria2::pauseDownload((Session*) session.ptr, gid, force);
        }

        int unpauseDownload(SessionHandle session, A2Gid gid) {
            return aria2::unpauseDownload((Session*) session.ptr, gid);
        }

        int changePosition(SessionHandle session, A2Gid gid, int pos, aria2::OffsetMode how) {
            return aria2::changePosition((Session*) session.ptr, gid, pos, how);
        }

        // </editor-fold>

        // Options
        // <editor-fold>

        int changeOption(SessionHandle session, A2Gid gid, const RKeyVals& rustOptions) {
            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            return aria2::changeOption((Session*) session.ptr, gid, options);
        }

        rust::Str getGlobalOption(SessionHandle session, const rust::Str name) {
            return rust::Str(aria2::getGlobalOption((Session*) session.ptr, std::string(name)));
        }

        RKeyVals getGlobalOptions(SessionHandle session) {
            aria2::KeyVals options(aria2::getGlobalOptions((Session*) session.ptr));
            RKeyVals rustOptions;
            __convertKeyValsBack(options, rustOptions);

            return rustOptions;
        }

        int changeGlobalOption(SessionHandle session, const RKeyVals& rustOptions) {
            aria2::KeyVals options;
            __convertKeyVals(rustOptions, options);

            return aria2::changeGlobalOption((Session*) session.ptr, options);
        }
        // </editor-fold>

        // Stats

        GlobalStat getGlobalStat(SessionHandle session) {
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

        RKeyVals DownloadHandleWrapper::getOptions() const {
            aria2::KeyVals options(handle->getOptions());
            RKeyVals dst;
            __convertKeyValsBack(options, dst);
            return dst;
        }

        std::unique_ptr<DownloadHandleWrapper> getDownloadHandle(SessionHandle session, A2Gid gid) {
            aria2::DownloadHandle* rawHandle = aria2::getDownloadHandle((Session*) session.ptr, gid);
            if (rawHandle != nullptr) {
                return std::make_unique<DownloadHandleWrapper>(rawHandle);
            } else {
                return std::unique_ptr<DownloadHandleWrapper>();
            }
        }

        void deleteDownloadHandle(std::unique_ptr<DownloadHandleWrapper> handle) {
            handle.reset();
        }

        // </editor-fold>

        // Internals
        // <editor-fold>

        int __eventCallbackDelegate(Session* session, DownloadEvent event, A2Gid gid, void* userData) {
            return EVENT_RUST_CALLBACK(
                    {.ptr = (size_t) session},
                    event,
                    gid,
                    (size_t) userData
            );
        }

        void __convertKeyVals(const RKeyVals& src, aria2::KeyVals& dst) {
            dst.reserve(src.size());
            for (auto item : src) {
                dst.push_back({std::string(item.key), std::string(item.val)});
            }
        }

        void __convertKeyValsBack(const aria2::KeyVals& src, RKeyVals& dst) {
            dst.reserve(src.size());
            for (auto item : src) {
                dst.push_back(
                        {
                                .key = rust::String(item.first),
                                .val = rust::String(item.second),
                        }
                );
            }
        }

        // </editor-fold>
    }
}
