use ninpo::connect::Connect;
use ninpo::domain::Domain;
use virt_sys;

fn main() {
    let uri: &str = "qemu:///session";
    let xml: &str = "
<domain type='kvm'>
  <name>ubuntu</name>
  <uuid>c9dbfecb-48ca-4bd8-84e8-96cb53460b5b</uuid>
  <metadata>
    <libosinfo:libosinfo xmlns:libosinfo=\"http://libosinfo.org/xmlns/libvirt/domain/1.0\">
      <libosinfo:os id=\"http://ubuntu.com/ubuntu/22.04\"/>
    </libosinfo:libosinfo>
  </metadata>
  <memory unit='KiB'>2097152</memory>
  <currentMemory unit='KiB'>2097152</currentMemory>
  <vcpu placement='static'>2</vcpu>
  <os firmware='efi'>
    <type arch='aarch64' machine='virt-7.2'>hvm</type>
    <boot dev='hd'/>
  </os>
  <features>
    <acpi/>
    <gic version='2'/>
  </features>
  <cpu mode='host-passthrough' check='none'/>
  <clock offset='utc'/>
  <on_poweroff>destroy</on_poweroff>
  <on_reboot>restart</on_reboot>
  <on_crash>destroy</on_crash>
  <devices>
    <emulator>/usr/bin/qemu-system-aarch64</emulator>
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2' discard='unmap'/>
      <source file='/home/rpi/ubuntu.qcow2'/>
      <target dev='vda' bus='virtio'/>
      <address type='pci' domain='0x0000' bus='0x05' slot='0x00' function='0x0'/>
    </disk>
    <disk type='file' device='cdrom'>
      <driver name='qemu' type='raw'/>
      <target dev='sda' bus='scsi'/>
      <readonly/>
      <address type='drive' controller='0' bus='0' target='0' unit='0'/>
    </disk>
    <controller type='usb' index='0' model='qemu-xhci' ports='15'>
      <address type='pci' domain='0x0000' bus='0x02' slot='0x00' function='0x0'/>
    </controller>
    <controller type='scsi' index='0' model='virtio-scsi'>
      <address type='pci' domain='0x0000' bus='0x03' slot='0x00' function='0x0'/>
    </controller>
    <controller type='pci' index='0' model='pcie-root'/>
    <controller type='pci' index='1' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='1' port='0x8'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x0' multifunction='on'/>
    </controller>
    <controller type='pci' index='2' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='2' port='0x9'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x1'/>
    </controller>
    <controller type='pci' index='3' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='3' port='0xa'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x2'/>
    </controller>
    <controller type='pci' index='4' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='4' port='0xb'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x3'/>
    </controller>
    <controller type='pci' index='5' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='5' port='0xc'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x4'/>
    </controller>
    <controller type='pci' index='6' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='6' port='0xd'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x5'/>
    </controller>
    <controller type='pci' index='7' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='7' port='0xe'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x6'/>
    </controller>
    <controller type='pci' index='8' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='8' port='0xf'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x01' function='0x7'/>
    </controller>
    <controller type='pci' index='9' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='9' port='0x10'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x0' multifunction='on'/>
    </controller>
    <controller type='pci' index='10' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='10' port='0x11'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x1'/>
    </controller>
    <controller type='pci' index='11' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='11' port='0x12'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x2'/>
    </controller>
    <controller type='pci' index='12' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='12' port='0x13'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x3'/>
    </controller>
    <controller type='pci' index='13' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='13' port='0x14'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x4'/>
    </controller>
    <controller type='pci' index='14' model='pcie-root-port'>
      <model name='pcie-root-port'/>
      <target chassis='14' port='0x15'/>
      <address type='pci' domain='0x0000' bus='0x00' slot='0x02' function='0x5'/>
    </controller>
    <controller type='virtio-serial' index='0'>
      <address type='pci' domain='0x0000' bus='0x04' slot='0x00' function='0x0'/>
    </controller>
    <interface type='bridge'>
      <mac address='52:54:00:3f:41:ac'/>
      <source bridge='br0'/>
      <model type='virtio'/>
      <address type='pci' domain='0x0000' bus='0x01' slot='0x00' function='0x0'/>
    </interface>
    <serial type='pty'>
      <target type='system-serial' port='0'>
        <model name='pl011'/>
      </target>
    </serial>
    <console type='pty'>
      <target type='serial' port='0'/>
    </console>
    <channel type='unix'>
      <target type='virtio' name='org.qemu.guest_agent.0'/>
      <address type='virtio-serial' controller='0' bus='0' port='1'/>
    </channel>
    <tpm model='tpm-tis'>
      <backend type='emulator' version='2.0'/>
    </tpm>
    <audio id='1' type='none'/>
    <memballoon model='virtio'>
      <address type='pci' domain='0x0000' bus='0x06' slot='0x00' function='0x0'/>
    </memballoon>
    <rng model='virtio'>
      <backend model='random'>/dev/urandom</backend>
      <address type='pci' domain='0x0000' bus='0x07' slot='0x00' function='0x0'/>
    </rng>
  </devices>
</domain>
";
    let id: &str = "ubuntu";

    if let Ok(mut conn) = Connect::open(Some(uri)) {
      println!("connect");

//      if let Ok(_dom) = Domain::define_xml(&conn, xml) {
//        println!("define");
//      }
//
//      if let Ok(dom) = Domain::lookup_by_name(&conn, id) {
//        assert_eq!(Ok(0), dom.create());
//        println!("start");
//      }

//      if let Ok(dom) = Domain::lookup_by_name(&conn, id) {
//        assert_eq!(Ok(0), dom.shutdown()); 
//        println!("shutdown");
//      }

//      if let Ok(dom) = Domain::lookup_by_name(&conn, id) {
//        assert_eq!(Ok(()), dom.undefine_flags(virt_sys::VIR_DOMAIN_UNDEFINE_NVRAM));
//        println!("undefine");
//      }

      assert_eq!(Ok(0), conn.close());
    }
}
