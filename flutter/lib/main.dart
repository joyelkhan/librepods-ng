import 'package:flutter/material.dart';

void main() {
  runApp(const LibrePodsApp());
}

class LibrePodsApp extends StatelessWidget {
  const LibrePodsApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'LibrePods NG',
      theme: ThemeData(useMaterial3: true, colorSchemeSeed: Colors.teal),
      home: const DeviceListScreen(),
    );
  }
}

class DeviceListScreen extends StatefulWidget {
  const DeviceListScreen({super.key});

  @override
  State<DeviceListScreen> createState() => _DeviceListScreenState();
}

class _DeviceListScreenState extends State<DeviceListScreen> {
  List<String> devices = [];
  bool isScanning = false;

  void _startScan() {
    setState(() => isScanning = true);
    Future.delayed(const Duration(seconds: 2), () {
      setState(() {
        devices = ['AirPods Pro 2', 'AirPods Max'];
        isScanning = false;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('LibrePods NG')),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: ElevatedButton.icon(
              onPressed: isScanning ? null : _startScan,
              icon: const Icon(Icons.bluetooth_searching),
              label: Text(isScanning ? 'Scanning...' : 'Scan Devices'),
            ),
          ),
          Expanded(
            child: ListView.builder(
              itemCount: devices.length,
              itemBuilder: (context, index) => ListTile(
                leading: const Icon(Icons.headphones),
                title: Text(devices[index]),
                trailing: const Icon(Icons.arrow_forward),
                onTap: () => _showDeviceDetails(devices[index]),
              ),
            ),
          ),
        ],
      ),
    );
  }

  void _showDeviceDetails(String device) {
    showModalBottomSheet(
      context: context,
      builder: (context) => Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(device, style: Theme.of(context).textTheme.headlineSmall),
            const SizedBox(height: 16),
            ListTile(
              title: const Text('Battery'),
              subtitle: const Text('85%'),
              leading: const Icon(Icons.battery_full),
            ),
            ListTile(
              title: const Text('ANC Mode'),
              subtitle: const Text('Active'),
              leading: const Icon(Icons.volume_off),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('Close'),
            ),
          ],
        ),
      ),
    );
  }
}
