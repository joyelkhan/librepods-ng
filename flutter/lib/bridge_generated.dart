import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

typedef StartScanNative = Int32 Function();
typedef StartScanDart = int Function();

typedef GetDevicesNative = Pointer<Utf8> Function();
typedef GetDevicesDart = Pointer<Utf8> Function();

typedef ConnectDeviceNative = Int32 Function(Pointer<Utf8> id);
typedef ConnectDeviceDart = int Function(Pointer<Utf8> id);

typedef SetAncModeNative = Int32 Function(Pointer<Utf8> id, Int32 mode);
typedef SetAncModeDart = int Function(Pointer<Utf8> id, int mode);

class FlutterBridge {
  static late DynamicLibrary _lib;
  static late StartScanDart _startScan;
  static late GetDevicesDart _getDevices;
  static late ConnectDeviceDart _connectDevice;
  static late SetAncModeDart _setAncMode;

  static Future<void> init() async {
    final String libName = Platform.isWindows
        ? 'rivers_ffi.dll'
        : Platform.isMacOS
            ? 'librivers_ffi.dylib'
            : 'librivers_ffi.so';
    _lib = DynamicLibrary.open(libName);
    _startScan = _lib.lookupFunction<StartScanNative, StartScanDart>('start_scan');
    _getDevices = _lib.lookupFunction<GetDevicesNative, GetDevicesDart>('get_devices');
    _connectDevice = _lib.lookupFunction<ConnectDeviceNative, ConnectDeviceDart>('connect_device');
    _setAncMode = _lib.lookupFunction<SetAncModeNative, SetAncModeDart>('set_anc_mode');
  }

  static int startScan() => _startScan();

  static String getDevices() {
    final ptr = _getDevices();
    final result = ptr.toDartString();
    malloc.free(ptr);
    return result;
  }

  static int connectDevice(String id) {
    final idPtr = id.toNativeUtf8();
    final result = _connectDevice(idPtr);
    malloc.free(idPtr);
    return result;
  }

  static int setAncMode(String id, int mode) {
    final idPtr = id.toNativeUtf8();
    final result = _setAncMode(idPtr, mode);
    malloc.free(idPtr);
    return result;
  }
}
