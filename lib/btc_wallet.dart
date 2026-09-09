import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_btc_wallet/src/rust/api/wrapper.dart';
import 'package:flutter_btc_wallet/error_dialog_helper.dart';

class BtcWalletPage extends StatefulWidget {
  const BtcWalletPage({super.key, required this.dataPath});

  final String dataPath;

  @override
  State<BtcWalletPage> createState() => _BtcWalletPageState();
}

class _BtcWalletPageState extends State<BtcWalletPage> {
  WalletWrapper? _wallet;
  BigInt _balance = BigInt.from(0);

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          // Balance
          BigCard(balance: _balance),
          SizedBox(height: 10),

          // Create Wallet
          ElevatedButton(
            onPressed: () async {
              onCreateWallet(context);
            },
            child: Text("Create Wallet"),
          ),
          SizedBox(height: 10),

          // Load Wallet
          ElevatedButton(
            onPressed: () async {
              onLoadWallet(context);
            },
            child: Text("Load Wallet"),
          ),
          SizedBox(height: 10),

          // Update balance
          ElevatedButton(
            onPressed: () async {
              onUpdateBalance(context);
            },
            child: Text("Update Balance"),
          ),
          SizedBox(height: 10),

          // Create new address
          ElevatedButton(
            onPressed: () async {
              onNewAddress(context);
            },
            child: Text("New Address"),
          ),
          SizedBox(height: 10),

          //
          Text(_wallet != null ? 'Wallet ready' : 'No wallet'),
        ],
      ),
    );
  }

  Future<void> onCreateWallet(BuildContext context) async {
    final walletPath = '${widget.dataPath}/wallet.db';
    final file = File(walletPath);
    if (await file.exists()) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'Already wallet file exists',
        );
      }
      return;
    }

    try {
      final wallet = await WalletWrapper.createWallet(
        network: 'regtest',
        electrumServer: 'tcp://192.168.0.41:50001',
        passphrase: 'abcdefg12345',
        walletPath: walletPath,
      );
      setState(() {
        _wallet = wallet;
      });
      debugPrint('wallet created: $walletPath');
    } catch (e) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'createWallet error: $e',
        );
      }
      debugPrint('createWallet error: $e');
      if (await file.exists()) {
        debugPrint('Remove wallet file: $walletPath');
        await file.delete();
      }
    }
  }

  Future<void> onLoadWallet(BuildContext context) async {
    if (_wallet != null) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'Already wallet loaded',
        );
      }
      return;
    }
    final walletPath = '${widget.dataPath}/wallet.db';

    try {
      final wallet = await WalletWrapper.loadWallet(
        passphrase: 'abcdefg12345',
        walletPath: walletPath,
      );
      final balance = await wallet.balance();
      setState(() {
        _wallet = wallet;
        _balance = balance;
      });
      debugPrint('wallet created: $walletPath');
    } catch (e) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'loadWallet error: $e',
        );
      }
      debugPrint('loadWallet error: $e');
    }
  }

  Future<void> onNewAddress(BuildContext context) async {
    if (_wallet == null) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'No loaded wallet',
        );
      }
      return;
    }
    try {
      final address = await _wallet!.newAddress();
      debugPrint('new address: $address');
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          title: 'New Address',
          message: address,
        );
      }
    } catch (e) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'newAddress error: $e',
        );
      }
      debugPrint('newAddress error: $e');
    }
  }

  Future<void> onUpdateBalance(BuildContext context) async {
    if (_wallet == null) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'No loaded wallet',
        );
      }
      return;
    }
    try {
      final balance = await _wallet!.balance();
      setState(() {
        _balance = balance;
      });
      debugPrint('balance: $balance');
    } catch (e) {
      if (context.mounted) {
        await ErrorHelper.showAlertDialog(
          context,
          message: 'balance error: $e',
        );
      }
      debugPrint('balance error: $e');
    }
  }
}

class BigCard extends StatelessWidget {
  const BigCard({super.key, required this.balance});

  final BigInt balance;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    // ↓ Add this.
    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
    );

    return Card(
      color: theme.colorScheme.primary,
      child: Padding(
        padding: const EdgeInsets.all(20),
        // ↓ Change this line.
        child: Text(balance.toString(), style: style),
      ),
    );
  }
}
