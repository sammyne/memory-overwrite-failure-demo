# Build Redis as follows:
#
# - make               -- create non-SGX no-debug-log manifest
# - make SGX=1         -- create SGX no-debug-log manifest
# - make SGX=1 DEBUG=1 -- create SGX debug-log manifest
#
# Any of these invocations clones Redis' git repository and builds Redis in
# default configuration and in the latest-to-date (5.0.5) version.
#
# By default, Redis uses poll/epoll mechanism of Linux. To build Redis with
# select, use `make USE_SELECT=1`. For correct re-builds, always clean up
# Redis source code beforehand via `make distclean`.
#
# Use `make clean` to remove Graphene-generated files and `make distclean` to
# additionally remove the cloned Redis git repository.

################################# CONSTANTS ###################################

# Relative path to Graphene root
SGX_SIGNER_KEY ?= $(HOME)/.gramine/developer-key.pem

ifeq ($(DEBUG),1)
GRAPHENE_LOG_LEVEL = debug
else
GRAPHENE_LOG_LEVEL = error
endif

.PHONY: all
all: hello-world.manifest
ifeq ($(SGX),1)
all: hello-world.manifest.sgx hello-world.sig hello-world.token
endif

ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

################################ REDIS MANIFEST ###############################

# The template file contains almost all necessary information to run Redis
# under Graphene / Graphene-SGX. We create redis.manifest (to be run under
# non-SGX Graphene) by simply replacing variables in the template file via sed.

hello-world.manifest: hello-world.manifest.template
	gramine-manifest \
		-Dentrypoint=hello-world \
		-Dlog_level=$(GRAPHENE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		$< > $@

# Manifest for Graphene-SGX requires special "gramine-sgx-sign" procedure. This procedure measures
# all Redis trusted files, adds the measurement to the resulting manifest.sgx file (among other,
# less important SGX options) and creates hello-world.sig (SIGSTRUCT object).
#
# Graphene-SGX requires EINITTOKEN and SIGSTRUCT objects (see SGX hardware ABI, in particular EINIT
# instruction). The "gramine-sgx-get-token" script generates EINITTOKEN based on a SIGSTRUCT and
# puts it in .token file. Note that filenames must be the same as the manifest name (i.e.,
# "hello-world"). EINITTOKEN must be generated on the machine where the application will run, not
# where it was built.

# Make on Ubuntu <= 20.04 doesn't support "Rules with Grouped Targets" (`&:`),
# we need to hack around.
hello-world.sig hello-world.manifest.sgx: sgx_outputs
	@:

.INTERMEDIATE: sgx_outputs
sgx_outputs: hello-world.manifest
	gramine-sgx-sign \
		--key $(SGX_SIGNER_KEY) \
		--manifest hello-world.manifest \
		--output hello-world.manifest.sgx

hello-world.token: hello-world.sig
	gramine-sgx-get-token --output $@ --sig $<

################################## CLEANUP ####################################

.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest hello-world *.rdb

.PHONY: distclean
distclean: clean
