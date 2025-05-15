#include "kernel/yosys.h"

YOSYS_NAMESPACE_BEGIN
namespace RTLIL {

RTLIL::Module *addModule(RTLIL::Design &design, const RTLIL::IdString &name) {
  return design.addModule(name);
}

RTLIL::Module *addModuleStr(RTLIL::Design &design, const std::string &name) {
  return design.addModule(name);
}

RTLIL::Wire *addWire(RTLIL::Module *module, const RTLIL::IdString &name,
                     int32_t width) {
  return module->addWire(name, width);
}

RTLIL::Wire *addWireStr(RTLIL::Module *module, const std::string &name,
                        int32_t width) {
  return module->addWire(name, width);
}

void setPortInput(RTLIL::Wire *w, bool b) { w->port_input = b; }

void setPortOutput(RTLIL::Wire *w, bool b) { w->port_output = b; }

void setPortId(RTLIL::Wire *w, int32_t id) { w->port_id = id; }

RTLIL::Cell *addNeg(RTLIL::Module *module, const RTLIL::IdString &name,
                    std::unique_ptr<RTLIL::SigSpec> sig_a,
                    std::unique_ptr<RTLIL::SigSpec> sig_y, bool is_signed,
                    const std::string &src) {
  return module->addNeg(name, *sig_a, *sig_y, is_signed, src);
}

RTLIL::Cell *addMux(RTLIL::Module *module, const RTLIL::IdString &name,
                    std::unique_ptr<RTLIL::SigSpec> sig_a,
                    std::unique_ptr<RTLIL::SigSpec> sig_b,
                    std::unique_ptr<RTLIL::SigSpec> sig_s,
                    std::unique_ptr<RTLIL::SigSpec> sig_y,
                    const std::string &src) {
  return module->addMux(name, *sig_a, *sig_b, *sig_s, *sig_y, src);
}

RTLIL::Cell *addNegStr(RTLIL::Module *module, const std::string &name,
                       std::unique_ptr<RTLIL::SigSpec> sig_a,
                       std::unique_ptr<RTLIL::SigSpec> sig_y, bool is_signed,
                       const std::string &src) {
  return module->addNeg(name, *sig_a, *sig_y, is_signed, src);
}

RTLIL::Cell *addMuxStr(RTLIL::Module *module, const std::string &name,
                       std::unique_ptr<RTLIL::SigSpec> sig_a,
                       std::unique_ptr<RTLIL::SigSpec> sig_b,
                       std::unique_ptr<RTLIL::SigSpec> sig_s,
                       std::unique_ptr<RTLIL::SigSpec> sig_y,
                       const std::string &src) {
  return module->addMux(name, *sig_a, *sig_b, *sig_s, *sig_y, src);
}

std::unique_ptr<RTLIL::SigSpec> makeSigSpec(RTLIL::Wire *wire, int32_t offset,
                                            int32_t width) {
  return std::make_unique<RTLIL::SigSpec>(wire, offset, width);
}

void fixup_ports(RTLIL::Module *module) {
  module->fixup_ports();
}

} // namespace RTLIL
YOSYS_NAMESPACE_END
